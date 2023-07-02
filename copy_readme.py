# Prepends README.md to lib.rs 

print('Copying start')

with open('./src/lib.rs', 'r') as lib_file:
    lines = [line for line in lib_file if not line.startswith('//!')]

with open('./src/lib.rs', 'w') as lib_file:
    for line in open('./README.md').readlines()[2:]:
        lib_file.write(f'//! {line}')

    for line in lines:
        lib_file.write(line)

print('Copying complete')
