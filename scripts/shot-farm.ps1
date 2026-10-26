Add-Type -AssemblyName System.Drawing, System.Windows.Forms
Add-Type -TypeDefinition @"
using System;
using System.Text;
using System.Runtime.InteropServices;
public class FarmShot {
  public delegate bool CB(IntPtr h, IntPtr l);
  [DllImport("user32.dll")] public static extern bool EnumWindows(CB cb, IntPtr l);
  [DllImport("user32.dll")] public static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint p);
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out R r);
  [DllImport("user32.dll")] public static extern bool SetWindowPos(IntPtr h, IntPtr a, int x, int y, int cx, int cy, uint f);
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int c);
  [DllImport("user32.dll")] public static extern bool BringWindowToTop(IntPtr h);
  [DllImport("user32.dll")] public static extern bool PrintWindow(IntPtr hwnd, IntPtr hdcBlt, uint nFlags);
  [DllImport("user32.dll")] public static extern bool AllowSetForegroundWindow(int pid);
  [StructLayout(LayoutKind.Sequential)] public struct R { public int L,T,Right,Bottom; }
}
"@

$proc = Get-Process deskpet-farm -ErrorAction Stop | Select-Object -First 1
$appPid = [uint32]$proc.Id
$script:farm = [IntPtr]::Zero
[FarmShot]::EnumWindows({
  param($h, $l)
  $p = [uint32]0
  [void][FarmShot]::GetWindowThreadProcessId($h, [ref]$p)
  if ($p -eq $appPid -and [FarmShot]::IsWindowVisible($h)) {
    $sb = New-Object Text.StringBuilder 64
    [void][FarmShot]::GetWindowText($h, $sb, 64)
    if ($sb.ToString() -eq "deskpet farm") { $script:farm = $h }
  }
  return $true
}, [IntPtr]::Zero) | Out-Null

if ($script:farm -eq [IntPtr]::Zero) { throw "farm window missing" }

[FarmShot]::AllowSetForegroundWindow(-1) | Out-Null
[FarmShot]::ShowWindow($script:farm, 9) | Out-Null
# Temporarily topmost so CopyFromScreen is not occluded by Chrome/Cursor
[FarmShot]::SetWindowPos($script:farm, [IntPtr]::new(-1), 40, 40, 1024, 780, 0x0040) | Out-Null
Start-Sleep -Milliseconds 450
[FarmShot]::BringWindowToTop($script:farm) | Out-Null
[FarmShot]::SetForegroundWindow($script:farm) | Out-Null
Start-Sleep -Milliseconds 350

$r = New-Object FarmShot+R
[FarmShot]::GetWindowRect($script:farm, [ref]$r) | Out-Null
$w = $r.Right - $r.L
$h = $r.Bottom - $r.T
$path = Join-Path $env:TEMP "farm-check.png"

function Score-Green([Drawing.Bitmap]$img) {
  $g = 0
  for ($x = 60; $x -lt $img.Width - 60; $x += 12) {
    for ($y = 120; $y -lt [Math]::Min(560, $img.Height - 40); $y += 12) {
      $c = $img.GetPixel($x, $y)
      if ($c.G -gt ($c.R + 20) -and $c.G -gt 90) { $g++ }
    }
  }
  return $g
}

# Prefer PrintWindow(PW_RENDERFULLCONTENT) — reliable for WebView2 even when occluded
$pwBmp = New-Object Drawing.Bitmap $w, $h
$pwG = [Drawing.Graphics]::FromImage($pwBmp)
$hdc = $pwG.GetHdc()
$pwOk = [FarmShot]::PrintWindow($script:farm, $hdc, 2)
$pwG.ReleaseHdc($hdc)
$pwG.Dispose()
$pwScore = if ($pwOk) { Score-Green $pwBmp } else { 0 }

# Fallback: CopyFromScreen
$bounds = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
$full = New-Object Drawing.Bitmap $bounds.Width, $bounds.Height
$sg = [Drawing.Graphics]::FromImage($full)
$sg.CopyFromScreen($bounds.Location, [Drawing.Point]::Empty, $bounds.Size)
$sg.Dispose()
$rect = [Drawing.Rectangle]::FromLTRB($r.L, $r.T, $r.Right, $r.Bottom)
$screenBmp = $full.Clone($rect, $full.PixelFormat)
$full.Dispose()
$scScore = Score-Green $screenBmp

if ($pwScore -ge $scScore -and $pwScore -gt 80) {
  $pwBmp.Save($path, [Drawing.Imaging.ImageFormat]::Png)
  $chosen = "printwindow"
  $score = $pwScore
} else {
  $screenBmp.Save($path, [Drawing.Imaging.ImageFormat]::Png)
  $chosen = "screen"
  $score = $scScore
}
$pwBmp.Dispose()
$screenBmp.Dispose()

# Drop topmost
[FarmShot]::SetWindowPos($script:farm, [IntPtr]::new(-2), 40, 40, 1024, 780, 0x0040) | Out-Null

Write-Output "size=${w}x${h} green=$score via=$chosen path=$path rect=$($r.L),$($r.T)-$($r.Right),$($r.Bottom)"
