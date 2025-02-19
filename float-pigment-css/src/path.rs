use alloc::string::String;

pub(crate) fn resolve(base: &str, rel: &str) -> String {
    let mut slices = vec![];
    let mut extra_parent_count = 0;
    let from_root = base.starts_with('/') || rel.starts_with('/');
    let main = if let Some(rel) = rel.strip_prefix('/') {
        rel
    } else {
        for slice in base.split('/') {
            match slice {
                "" | "." => {}
                ".." => {
                    if slices.pop().is_none() && !from_root {
                        extra_parent_count += 1;
                    }
                }
                _ => {
                    slices.push(slice);
                }
            }
        }
        rel
    };
    slices.pop();
    for slice in main.split('/') {
        match slice {
            "" | "." => {}
            ".." => {
                if slices.pop().is_none() && !from_root {
                    extra_parent_count += 1;
                }
            }
            _ => {
                slices.push(slice);
            }
        }
    }
    let mut ret = String::new();
    for _ in 0..extra_parent_count {
        ret += "../";
    }
    ret += &slices.join("/");
    ret
}
