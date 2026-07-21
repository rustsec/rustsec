//! CVSS v3.1 Base Metric Group

mod a;
pub use a::Availability;

mod ac;
pub use ac::AttackComplexity;

mod av;
pub use av::AttackVector;

mod c;
pub use c::Confidentiality;

mod i;
pub use i::Integrity;

mod pr;
pub use pr::PrivilegesRequired;

mod s;
pub use s::Scope;

mod ui;
pub use ui::UserInteraction;
