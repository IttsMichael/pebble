use crate::models::Package;

/// Calculates a relevance score for a single package against the current search query.
/// Higher score means more relevant.
pub fn calculate_score(query: &str, pkg: &Package) -> i32 {
    let q = query.to_lowercase();
    let name = pkg.name.to_lowercase();
    let name_only = name.split_whitespace().next().unwrap_or(&name);
    let desc = pkg.description.to_lowercase();

    let mut score = 0;

    // 1. Exact Name Match (Highest Priority)
    if name_only == q {
        score += 1000;
    }
    // 2. Starts With (High Priority)
    else if name_only.starts_with(&q) {
        score += 500;
    }
    // 3. Contains in Name (Medium Priority)
    else if name_only.contains(&q) {
        score += 100;
    }
    // 4. Contains in Description (Lowest Priority)
    else if desc.contains(&q) {
        score += 10;
    }

    score
}

/// Consumes a raw list of packages from pacman, scores them against the query,
/// and returns them perfectly sorted by relevance -> length -> alphabetical.
pub fn sort_packages(query: &str, mut packages: Vec<Package>) -> Vec<Package> {
    if query.trim().is_empty() {
        return packages;
    }

    packages.sort_by(|a, b| {
        let score_a = calculate_score(query, a);
        let score_b = calculate_score(query, b);

        // Sort descending by score
        if score_a != score_b {
            return score_b.cmp(&score_a);
        }

        // Tie-breaker 1: Shorter names are usually the actual package (e.g. `linux` vs `linux-headers`)
        let name_a = a.name.split_whitespace().next().unwrap_or(&a.name);
        let name_b = b.name.split_whitespace().next().unwrap_or(&b.name);
        
        if name_a.len() != name_b.len() {
            return name_a.len().cmp(&name_b.len());
        }

        // Tie-breaker 2: Alphabetical fallback
        name_a.cmp(name_b)
    });

    // Only return the top 10 most relevant results for performance
    packages.truncate(10);

    packages
}
