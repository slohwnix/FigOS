# generate_keymap.py
# 256 normal + 256 shift + 256 special (extended)
keymap = [0] * 768


def set_key(scancode, normal, shift, special=None):
    keymap[scancode] = ord(normal) if isinstance(normal, str) else normal
    keymap[scancode + 256] = ord(shift) if isinstance(shift, str) else shift
    if special:
        keymap[scancode + 512] = ord(special) if isinstance(special, str) else special


# Chiffres et symboles (Ligne du haut)
set_key(0x02, "&", "1")
set_key(0x03, "é", "2")
set_key(0x04, '"', "3")
set_key(0x05, "'", "4")
set_key(0x06, "(", "5")
set_key(0x07, "-", "6")
set_key(0x08, "è", "7")
set_key(0x09, "_", "8")
set_key(0x0A, "ç", "9")
set_key(0x0B, "à", "0")
set_key(0x0C, ")", "°")

# Lettres
letters = {
    0x10: "a",
    0x11: "z",
    0x12: "e",
    0x13: "r",
    0x14: "t",
    0x15: "y",
    0x16: "u",
    0x17: "i",
    0x18: "o",
    0x19: "p",
    0x1E: "q",
    0x1F: "s",
    0x20: "d",
    0x21: "f",
    0x22: "g",
    0x23: "h",
    0x24: "j",
    0x25: "k",
    0x26: "l",
    0x27: "m",
    0x2C: "w",
    0x2D: "x",
    0x2E: "c",
    0x2F: "v",
    0x30: "b",
    0x31: "n",
}
for scan, char in letters.items():
    set_key(scan, char, char.upper())

# Pavé numérique ET Touches Spéciales (Extended)
# Pour les flèches, on utilise nos codes custom \x11 et \x12 dans la 3ème colonne
set_key(0x47, "7", "7")
set_key(0x48, "8", "8", special="\x11")  # Up
set_key(0x49, "9", "9")
set_key(0x4B, "4", "4")
set_key(0x4C, "5", "5")
set_key(0x4D, "6", "6")
set_key(0x4F, "1", "1")
set_key(0x50, "2", "2", special="\x12")  # Down
set_key(0x51, "3", "3")
set_key(0x52, "0", "0")
set_key(0x53, ".", ".")

# Ponctuation
set_key(0x32, ",", "?")
set_key(0x33, ";", ".")
set_key(0x34, ":", "/")
set_key(0x35, "!", "§")
set_key(0x39, " ", " ")

with open("keymap.bin", "wb") as f:
    f.write(bytearray(keymap))
