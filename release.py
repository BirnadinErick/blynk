import os
import shutil
from zipfile import ZipFile

BASE_DIR:str = os.path.dirname(__file__)
BIN_PATH:str = os.path.join(BASE_DIR, "target", "release", "blynk.exe")
LICENSE_FILE:str = "LICENSE.md"

print("BASE DIR: ", BASE_DIR )
print("BINARY PATH: ", BIN_PATH)

print("moving release binary...")
shutil.copyfile(BIN_PATH, os.path.join(BASE_DIR, "blynk"))
print("done moving!")

print("zipping...")
with ZipFile('blynk_latest.zip', 'w') as zip:
    zip.write("blynk.exe")
    zip.write("LICENSE.md")
print("Done zipping!")
