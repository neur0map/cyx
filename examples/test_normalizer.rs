use cyx::cache::QueryNormalizer;

fn main() -> anyhow::Result<()> {
    let normalizer = QueryNormalizer::with_defaults()?;

    let test_queries = vec![
        "Show me nmap SYN scan!!!",
        "show me how to do nmap syn scan",
        "NMAP SYN SCAN",
        "nmap stealth scan",
        "How do I use sqli for testing?",
        "Show me privesc techniques",
        "What is a reverse shell?",
        "nmap -sS scan",
    ];

    println!("Query Normalization Examples\n");
    println!("{:<45} -> Normalized", "Original");
    println!("{}", "=".repeat(90));

    for query in test_queries {
        let normalized = normalizer.normalize(query)?;
        let hash = normalizer.compute_hash(&normalized);
        println!("{:<45} -> {}", query, normalized);
        println!("{:<45}    Hash: {}\n", "", hash);
    }

    // Test that similar queries produce the same hash
    println!("\n{}", "=".repeat(90));
    println!("Hash Collision Test (similar queries should have same hash):\n");

    let similar_queries = vec![("Show me nmap scan", "NMAP SCAN", "nmap scan")];

    for (q1, q2, q3) in similar_queries {
        let n1 = normalizer.normalize(q1)?;
        let n2 = normalizer.normalize(q2)?;
        let n3 = normalizer.normalize(q3)?;

        let h1 = normalizer.compute_hash(&n1);
        let h2 = normalizer.compute_hash(&n2);
        let h3 = normalizer.compute_hash(&n3);

        println!("Query 1: \"{}\"", q1);
        println!("  Normalized: \"{}\"", n1);
        println!("  Hash: {}", h1);
        println!();
        println!("Query 2: \"{}\"", q2);
        println!("  Normalized: \"{}\"", n2);
        println!("  Hash: {}", h2);
        println!();
        println!("Query 3: \"{}\"", q3);
        println!("  Normalized: \"{}\"", n3);
        println!("  Hash: {}", h3);
        println!();

        if h1 == h2 && h2 == h3 {
            println!("✓ All hashes match! Queries will be treated as identical.\n");
        } else {
            println!("✗ Hashes differ! Queries will be treated as different.\n");
        }
    }

    Ok(())
}
