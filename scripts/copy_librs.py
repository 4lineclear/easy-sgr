# Prepends README.md to lib.rs

print('Copying start')

with open('./src/lib.rs', 'r') as readme_file:
    lines = [line for line in readme_file if line.startswith('//!')]

badges = (
    '[![Build status](https://github.com/4lineclear/easy-sgr/actions/workflows/rust.yml/badge.svg)](https://github.com/4lineclear/easy-sgr/actions) '
    '[![Crates.io](https://img.shields.io/crates/v/easy-sgr)](https://crates.io/crates/easy-sgr) '
    '[![docs.rs](https://img.shields.io/docsrs/easy-sgr)](https://docs.rs/easy-sgr) '
    '[![License](https://img.shields.io/crates/l/easy-sgr)](https://github.com/4lineclear/easy-sgr/blob/main/LICENSE) '
    '[![Code Coverage](https://codecov.io/gh/4lineclear/easy-sgr/branch/main/graph/badge.svg?token=0Q30XAW0PV)](https://codecov.io/gh/4lineclear/easy-sgr)'
)

with open('./README.md', 'w') as readme_file:
    readme_file.write(f'# easy-sgr\n\n{badges}\n\n')
    for line in lines:
        line = line[4:]
        if len(line) == 0:
            readme_file.write('\n')
        else:
            readme_file.write(line)

print('Copying complete')
