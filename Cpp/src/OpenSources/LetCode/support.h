#include <gtest/gtest-param-test.h>
#include <gtest/gtest.h>

namespace Support {
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