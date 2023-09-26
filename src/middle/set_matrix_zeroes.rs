/// # 73 矩阵置零 (set-matrix-zeroes)
///
///难度 ：Medium
///
///描述：<p>给定一个&nbsp;<code><em>m</em> x <em>n</em></code> 的矩阵，如果一个元素为 <strong>0 </strong>，则将其所在行和列的所有元素都设为 <strong>0</strong> 。请使用 <strong><a href="http://baike.baidu.com/item/%E5%8E%9F%E5%9C%B0%E7%AE%97%E6%B3%95" target="_blank">原地</a></strong> 算法<strong>。</strong></p>
///
///<ul>
///</ul>
///
///<p>&nbsp;</p>
///
///<p><strong>示例 1：</strong></p>
///<img alt="" src="https://assets.leetcode.com/uploads/2020/08/17/mat1.jpg" style="width: 450px; height: 169px;" />
///<pre>
///<strong>输入：</strong>matrix = [[1,1,1],[1,0,1],[1,1,1]]
///<strong>输出：</strong>[[1,0,1],[0,0,0],[1,0,1]]
///</pre>
///
///<p><strong>示例 2：</strong></p>
///<img alt="" src="https://assets.leetcode.com/uploads/2020/08/17/mat2.jpg" style="width: 450px; height: 137px;" />
///<pre>
///<strong>输入：</strong>matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
///<strong>输出：</strong>[[0,0,0,0],[0,4,5,0],[0,3,1,0]]
///</pre>
///
///<p>&nbsp;</p>
///
///<p><strong>提示：</strong></p>
///
///<ul>
///	<li><code>m == matrix.length</code></li>
///	<li><code>n == matrix[0].length</code></li>
///	<li><code>1 &lt;= m, n &lt;= 200</code></li>
///	<li><code>-2<sup>31</sup> &lt;= matrix[i][j] &lt;= 2<sup>31</sup> - 1</code></li>
///</ul>
///
///<p>&nbsp;</p>
///
///<p><strong>进阶：</strong></p>
///
///<ul>
///	<li>一个直观的解决方案是使用 &nbsp;<code>O(<em>m</em><em>n</em>)</code>&nbsp;的额外空间，但这并不是一个好的解决方案。</li>
///	<li>一个简单的改进方案是使用 <code>O(<em>m</em>&nbsp;+&nbsp;<em>n</em>)</code> 的额外空间，但这仍然不是最好的解决方案。</li>
///	<li>你能想出一个仅使用常量空间的解决方案吗？</li>
///</ul>
///
///
///url: https://leetcode-cn.com/problems/set-matrix-zeroes/description
#[cfg(test)]
mod tests {
    #[test]
    fn test_set_matrix_zeroes(){
        // let mut vec1 = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let mut vec1 = vec![vec![0,1,2,0], vec![3,4,5,2], vec![1,3,1,5]];
        let _ = set_zeroes(&mut vec1);
        println!("{:?}", vec1);
    }
    ///时间  0ms 击败 100.00%使用 Rust 的用户
    /// 内存 2.34MB 击败 27.08%使用 Rust 的用户
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut zero_index = vec![];
        for row in matrix.iter() {
            row.iter().enumerate().for_each(|(index,value)|{
                if value==&0 {
                    zero_index.push(index);
                }
            })
        }
        for row in matrix.iter_mut() {
            if row.contains(&0) {
                row.iter_mut().for_each(|x|{
                    *x = 0;
                })
            }else {
                for index in zero_index.iter() {
                    row[*index] = 0;
                }
            }
        }
    }
}
   