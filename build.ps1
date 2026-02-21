# DeepRant 构建脚本
# 自动设置 MSVC 环境并构建 Tauri 应用

$sdkVer = "10.0.26100.0"
$vsVer = "14.44.35207"
$sdkPath = "C:\Program Files (x86)\Windows Kits\10"
$vsPath = "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\$vsVer"

# 设置 MSVC 环境变量
$env:LIB = "$sdkPath\Lib\$sdkVer\um\x64;$sdkPath\Lib\$sdkVer\ucrt\x64;$vsPath\lib\x64"
$env:LIBPATH = "$sdkPath\UnionMetadata\$sdkVer;$sdkPath\References\$sdkVer;$vsPath\lib\x64"
$env:INCLUDE = "$sdkPath\Include\$sdkVer\um;$sdkPath\Include\$sdkVer\ucrt;$sdkPath\Include\$sdkVer\shared;$sdkPath\Include\$sdkVer\winrt;$vsPath\include"
$env:Path = "$vsPath\bin\Hostx64\x64;$sdkPath\bin\$sdkVer\x64;" + $env:Path

Write-Host "正在构建 DeepRant..." -ForegroundColor Green

# 执行构建
npm run tauri build

if ($LASTEXITCODE -eq 0) {
    Write-Host "`n✅ 构建成功！" -ForegroundColor Green
    Write-Host "安装包位置: src-tauri\target\release\bundle\msi\" -ForegroundColor Cyan
    
    # 打开安装包目录
    explorer.exe ".\src-tauri\target\release\bundle\msi"
} else {
    Write-Host "`n❌ 构建失败，退出码: $LASTEXITCODE" -ForegroundColor Red
}
