// This file is only used if you don't have the nessecary env variables set up. If you do have you can just run
// dx serve --platform android

$env:ANDROID_HOME     = "D:\Android" 
$env:ANDROID_SDK_ROOT = "D:\Android" 
$env:JAVA_HOME        = "C:\Program Files\Java\jdk-24"

Write-Host "ANDROID_HOME     = $env:ANDROID_HOME"
Write-Host "ANDROID_NDK_HOME = $env:ANDROID_NDK_HOME"
Write-Host "JAVA_HOME        = $env:JAVA_HOME"

dx serve --platform android
