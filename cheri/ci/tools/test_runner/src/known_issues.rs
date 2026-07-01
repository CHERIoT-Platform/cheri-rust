use std::path::Path;

#[derive(Default)]
pub struct KnownIssues {
    // Vec is a tad simpler than Set and we don't expect this to be large
    pub issues: Vec<(String, String)>,
}

//
impl KnownIssues {
    /// The known issues file should have one issue per line in the format `{test_name} {issue}`,
    /// where `test_name` is the qualified function name printed by the test runner, e.g.
    /// `num::i32::test_swap_bytes` and `issue` could be a link to an issue on GitHub (or any
    /// other relevant string, this is not enforced here, but we should keep to some convention).
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let issues = content
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|line| {
                let (name, issue) =
                    line.rsplit_once(' ').ok_or_else(|| anyhow::anyhow!("invalid line: {line}"))?;
                Ok((name.trim().to_string(), issue.trim().to_string()))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Self { issues })
    }

    pub fn get(&self, name: &String) -> Option<&String> {
        self.issues.iter().find(|(n, _)| n == name).map(|(_, issue)| issue)
    }
}
