# Based on: https://gist.github.com/angstwad/bf22d1822c38a92ec0a9
import collections


def dict_merge(dct: dict, merge_dct: dict):
    """ Recursive dict merge. Inspired by :meth:``dict.update()``, instead of
    updating only top-level keys, dict_merge recurses down into dicts nested
    to an arbitrary depth, updating keys. The ``merge_dct`` is merged into
    ``dct``.
    :param dct: dict onto which the merge is executed
    :param merge_dct: dct merged into dct
    :return: None
    """
    for k, v in merge_dct.items():
        if (k in dct and isinstance(dct[k], dict)
                and isinstance(merge_dct[k], collections.Mapping)):
            dict_merge(dct[k], merge_dct[k])
        else:
            dct[k] = merge_dct[k]


def _contains_dict(child: dict):
    for k, v in child.items():
        if isinstance(v, dict):
            return True
    return False


def dict_replace(dct: dict, replace_dct: dict):
    if len(replace_dct) == 0:
        raise ValueError("Replace dictionary cannot be empty")
    for k, v in replace_dct.items():
        if (k in dct and isinstance(dct[k], dict)
                and isinstance(replace_dct[k], collections.Mapping)):
            if _contains_dict(dct[k]) and _contains_dict(replace_dct[k]):
                dict_replace(dct[k], replace_dct[k])
            else:
                dct[k] = replace_dct[k]
        else:
            dct[k] = replace_dct[k]
