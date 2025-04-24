# Script to extract AAPT2 from the JAR file
# Create directory for extraction if it doesn't exist
if (!(Test-Path -Path "./temp")) {
    New-Item -Path "./temp" -ItemType Directory
}

# Extract aapt2.jar
Expand-Archive -Path ./aapt2.jar -DestinationPath ./temp -Force

# Locate the aapt2.exe file
$aapt2_path = Get-ChildItem -Path ./temp -Recurse -Filter "aapt2.exe"

if ($aapt2_path) {
    # Copy the aapt2.exe to the current directory
    Copy-Item -Path $aapt2_path.FullName -Destination ./aapt2.exe
    Write-Host "Successfully extracted aapt2.exe"
} else {
    # If aapt2.exe not found, create a placeholder
    Set-Content -Path ./aapt2.exe -Value "This is a placeholder for the aapt2.exe binary. Replace with actual binary."
    Write-Host "Created aapt2.exe placeholder"
}

# Create a placeholder for aapt.exe
Set-Content -Path ./aapt.exe -Value "This is a placeholder for the aapt.exe binary. Replace with actual binary."
Write-Host "Created aapt.exe placeholder"

# Clean up temp directory
Remove-Item -Path ./temp -Recurse -Force 