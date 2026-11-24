nasm -f bin boot/boot.asm -o boot.bin
if ($LASTEXITCODE -ne 0) { throw "NASM failed" }

$bootSector = [IO.File]::ReadAllBytes("boot.bin")
if ($bootSector.Length -ne 512) { throw "Expected a 512-byte boot sector" }

$image = [byte[]]::new(360 * 1024)
[Array]::Copy($bootSector, $image, $bootSector.Length)
[IO.File]::WriteAllBytes("boot.img", $image)

Remove-Item -LiteralPath "boot.bin" -ErrorAction Stop