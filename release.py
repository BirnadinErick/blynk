import os
import platform
import shutil
from zipfile import ZipFile

BASE_DIR:str = os.path.dirname(__file__)
BIN_NAME = "blynk.exe" if str(
    platform.system()).lower() == "windows" else "blynk"
BIN_PATH: str = os.path.join(BASE_DIR, "target", "release", BIN_NAME)
LICENSE_FILE:str = "LICENSE.md"

print("BASE DIR: ", BASE_DIR )
print("BINARY PATH: ", BIN_PATH)

print("moving release binary...")
shutil.copyfile(BIN_PATH, os.path.join(BASE_DIR, BIN_NAME))
print("done moving!")

print("zipping...")
with ZipFile('blynk_latest.zip', 'w') as zip:
    zip.write("blynk")
    zip.write("LICENSE.md")
print("Done zipping!")
