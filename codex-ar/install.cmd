@echo off
rem Adds this folder to your user PATH so "codex-ar" works in any terminal.
powershell -NoProfile -ExecutionPolicy Bypass -Command "$d=(Resolve-Path -LiteralPath '%~dp0').Path.TrimEnd('\'); $p=[Environment]::GetEnvironmentVariable('Path','User'); if(-not $p){$p=''}; if(($p -split ';') -notcontains $d){[Environment]::SetEnvironmentVariable('Path',($p.TrimEnd(';')+';'+$d).TrimStart(';'),'User')}; Write-Host 'Done. Open a NEW terminal and type: codex-ar'"
pause
