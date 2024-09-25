import subprocess
import shutil
import os

# build asm files
subprocess.run(['wasm-pack', 'build', '--target', 'web'])

# copy files over to web directory
src_dir = './pkg'
dest_dir = '../web'

files_to_copy = ['wasm.js', 'wasm_bg.wasm']

for file_name in files_to_copy:
    src_file = os.path.join(src_dir, file_name)
    dest_file = os.path.join(dest_dir, file_name)

    try:
        shutil.copy(src_file, dest_file)
        print(f'Copied {file_name} to {dest_dir}')
    except FileNotFoundError as e:
        print(f'Error copying {file_name}: {e}')
