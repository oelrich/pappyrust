import pappyrust as pr
import numpy as np


def test_sum_as_string():
    assert pr.sum_as_string(4, 2) == "Summan är: 6"


def test_multi_playah():
    values = np.array([1.0, 2.0, 3.0])
    pr.multi_playah(3.0, values)
    assert np.array_equal(values, np.array([3.0, 6.0, 9.0]))
