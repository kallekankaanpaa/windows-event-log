$prevPwd = $PWD; Set-Location -ErrorAction Stop -LiteralPath $PSScriptRoot
try {
    Copy-Item -Path "hooks\*" -Destination "..\.git\hooks" -Recurse
}
finally {
    $prevPwd | Set-Location
}
