#include <gtest/gtest-param-test.h>
#include <gtest/gtest.h>

namespace Support {
template <class Array>
void compareTwoArray(const Array& expected_array, const Array& real_array) {
    ASSERT_EQ(expected_array.size(), real_array.size());
    size_t index{0};
    for (auto it1 = expected_array.begin(), it2 = real_array.begin(); it1 != expected_array.end(); ++it1, ++it2, ++index) {
        ASSERT_EQ(*it1, *it2) << "index = " << index;
    }
}

template <class Matrix>
void compareTwoMatrix(const Matrix& expected_matrix, const Matrix& real_matrix) {
    ASSERT_EQ(expected_matrix.size(), real_matrix.size());
    for (size_t i = 0; i < expected_matrix.size(); ++i) {
        ASSERT_EQ(expected_matrix[i].size(), real_matrix[i].size());
        for (size_t j = 0; j < expected_matrix[i].size(); ++j) {
            ASSERT_EQ(expected_matrix[i][j], real_matrix[i][j]) << "i = " << i << ", j = " << j;
        }
    }
}
} // namespace Support