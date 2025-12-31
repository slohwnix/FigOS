import os
import subprocess
import tempfile
import shutil

#############################################
# CONFIG
EFI_BIN = "./deploy/ESP/EFI/BOOT/BOOTX64.EFI"
OUT_ISO = "uefi_boot.iso"
LABEL = "MYUEFIISO"
#############################################

tmp = tempfile.mkdtemp()
esp_img = os.path.join(tmp, "esp.img")

try:
    size_mb = 64
    with open(esp_img, "wb") as f:
        f.truncate(size_mb * 1024 * 1024)

    subprocess.check_call(["mkfs.vfat", "-F", "32", esp_img])

    subprocess.check_call(["mmd", "-i", esp_img, "::EFI"])
    subprocess.check_call(["mmd", "-i", esp_img, "::EFI/BOOT"])

    subprocess.check_call(["mcopy", "-i", esp_img, EFI_BIN, "::EFI/BOOT/BOOTX64.EFI"])

    iso_root = os.path.join(tmp, "root")
    os.makedirs(iso_root, exist_ok=True)

    shutil.copy(esp_img, os.path.join(iso_root, "efiboot.img"))

    cmd = [
        "xorriso",
        "-as",
        "mkisofs",
        "-o",
        OUT_ISO,
        "-V",
        LABEL,
        "-eltorito-alt-boot",
        "-e",
        "efiboot.img",
        "-no-emul-boot",
        "-isohybrid-gpt-basdat",
        iso_root,
    ]
    subprocess.check_call(cmd)

    print("ISO UEFI bootable créée :", OUT_ISO)

finally:
    shutil.rmtree(tmp)
