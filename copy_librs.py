# Prepends README.md to lib.rs 

print('Copying start')

with open('./src/lib.rs', 'r') as readme_file:
    lines = [line for line in readme_file if line.startswith('//!')]

with open('./README.md', 'w') as readme_file:
    readme_file.write('# easy-sgr\n\n[![Build status](https://github.com/4lineclear/easy-sgr/actions/workflows/rust.yml/badge.svg)](https://github.com/4lineclear/easy-sgr/actions)\n\n')
    for line in lines:
        line = line[4:]
        if len(line) == 0:
            readme_file.write('\n')
        else:
            readme_file.write(line)

print('Copying complete')
