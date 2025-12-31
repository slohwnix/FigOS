import os
import shutil
import sys
import subprocess


def main():
    if len(sys.argv) < 2:
        print("Usage: python runner.py <path_to_efi>")
        sys.exit(1)

    efi_path = sys.argv[1]
    root_dir = os.getcwd()
    deploy_dir = os.path.join(root_dir, "deploy")
    esp_dir = os.path.join(deploy_dir, "ESP")
    boot_dir = os.path.join(esp_dir, "EFI", "BOOT")

    if os.path.exists(deploy_dir):
        shutil.rmtree(deploy_dir)
    os.makedirs(boot_dir)

    shutil.copy(efi_path, os.path.join(root_dir, "FigOS.efi"))
    shutil.copy(efi_path, os.path.join(boot_dir, "BOOTX64.EFI"))

    ovmf_path = os.path.join(root_dir, "OVMF.fd")

    qemu_cmd = [
        "qemu-system-x86_64",
        "-bios",
        ovmf_path,
        "-drive",
        f"format=raw,file=fat:rw:{esp_dir}",
        "-m",
        "1G",
        "-device",
        "virtio-vga,xres=1920,yres=1080",
        "-net",
        "none",
        "-serial",
        "stdio",
        "-display",
        "sdl",
        "-d",
        "int,cpu_reset",
        "-D",
        "qemu.log",
        "-no-reboot",
    ]

    print(f"Starting QEMU with high resolution support")
    subprocess.run(qemu_cmd)


if __name__ == "__main__":
    main()
