//!Longest Common Substring
//!
pub mod lcs {
    /// find longest common substr
    /// ```rust
    /// use single_pass::lcs::longest_common_substr;
    /// assert_eq!(longest_common_substr("me", "do"), 0);
    /// assert_eq!(longest_common_substr("how", "hi"), 1);
    /// assert_eq!(longest_common_substr("doyou", "i do"), 2);
    /// ```
    pub fn longest_common_substr(x: &str, y: &str) -> i32 {
        let x = x.chars().collect::<Vec<char>>();
        let y = y.chars().collect::<Vec<char>>();
        let m = x.len();
        let n = y.len();
        let mut ans = 0;
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 0..=m {
            for j in 0..=n {
                if i == 0 || j == 0 {
                } else if x[i - 1] == y[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                    ans = std::cmp::max(ans, dp[i][j]);
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::lcs::*;

    #[test]
    fn test_lcs() {
        assert_eq!(longest_common_substr("GeeksforGeeks", "GeeksQuiz"), 5);
    }

    #[test]
    fn test_lcs_2() {
        assert_eq!(longest_common_substr("the", "he"), 2);
    }

    #[test]
    fn test_lcs_3() {
        assert_eq!(longest_common_substr("the", "you"), 0);
    }
}

// https://www.programcreek.com/2015/04/longest-common-substring-java/
// https://www.geeksforgeeks.org/longest-common-substring-space-optimized-dp-solution/

// python
// # Returns length of longest common
// # substring of X[0..m-1] and Y[0..n-1]
// def LCSubStr(X, Y, m, n):
//
// # Create a table to store lengths of
// # longest common suffixes of substrings.
// # Note that LCSuff[i][j] contains the
// # length of longest common suffix of
// # X[0...i-1] and Y[0...j-1]. The first
// # row and first column entries have no
// # logical meaning, they are used only
// # for simplicity of the program.
//
// # LCSuff is the table with zero
// # value initially in each cell
// LCSuff = [[0 for k in range(n+1)] for l in range(m+1)]
//
// # To store the length of
// # longest common substring
// result = 0
//
// # Following steps to build
// # LCSuff[m+1][n+1] in bottom up fashion
// for i in range(m + 1):
// for j in range(n + 1):
// if (i == 0 or j == 0):
// LCSuff[i][j] = 0
// elif (X[i-1] == Y[j-1]):
// LCSuff[i][j] = LCSuff[i-1][j-1] + 1
// result = max(result, LCSuff[i][j])
// else:
// LCSuff[i][j] = 0
// return result
