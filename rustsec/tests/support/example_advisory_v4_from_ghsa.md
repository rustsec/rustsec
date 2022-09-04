```toml
id = "RUSTSEC-2001-2101"
package = "base"
date = "2001-02-03"
url = "https://www.youtube.com/watch?v=jQE66WA2s-A"
categories = ["code-execution", "privilege-escalation"]
keywords = ["how", "are", "you", "gentlemen"]
aliases = ["CVE-2001-2101"]
cvss = "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:C/C:H/I:H/A:H"
license = "CC-BY-4.0"
attribution_url = "https://github.com/advisories/GHSA-f8vr-r385-rh5r"

[versions]
patched = [">= 1.2.3"]
unaffected = ["0.1.2"]

[affected]
arch = ["x86"]
os = ["windows"]
functions = { "base::belongs::All" = ["< 1.2.3"] }
```

# All your base are belong to us

You have no chance to survive. Make your time.
