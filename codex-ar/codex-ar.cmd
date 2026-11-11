@echo off
rem Unofficial Arabic build of Codex CLI. Runs Codex in the folder you launch it from.
chcp 65001 >nul
"%~dp0codex-package\bin\codex.exe" %*
