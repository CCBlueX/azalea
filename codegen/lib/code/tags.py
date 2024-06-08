from lib.utils import to_snake_case, upper_first_letter, get_dir_location, to_camel_case

REGISTRIES_DIR = get_dir_location('../azalea-registry/src/tags')


def generate_tags(registries: dict, file_name: str, struct_name: str):
    tags_dir = f'{REGISTRIES_DIR}/{file_name}.rs'

    generated = f'''// This file was generated by codegen/lib/code/tags.py, don't edit it manually!

use std::collections::HashSet;

use once_cell::sync::Lazy;

use crate::{struct_name};

'''

    for tag_name, tag in sorted(registries.items(), key=lambda x: x[0]):
        tag_name = tag_name.replace('/', '_')
        static_set_name = to_snake_case(tag_name).upper()
        generated += f'pub static {static_set_name}: Lazy<HashSet<{struct_name}>> = Lazy::new(|| HashSet::from_iter(vec!['

        queue = tag['values'].copy()
        while queue != []:
            item = queue.pop(0)
            namespace, item_name = item.split(':')
            if namespace[0] == '#':
                queue += registries[item_name]['values']
                continue
            generated += f'{struct_name}::{upper_first_letter(to_camel_case(item_name))},\n'
        generated += ']));\n'

    with open(tags_dir, 'w') as f:
        f.write(generated)