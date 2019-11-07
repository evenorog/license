use crate::*;
use core::fmt::{self, Display, Formatter};

/// Returns an extension license based on the provided id.
///
/// # Examples
/// ```
/// # use license::{MIT, License};
/// let mit = license::from_id_ext(MIT.id()).unwrap();
/// assert_eq!(mit.id(), MIT.id());
/// ```
#[inline]
pub fn from_id_ext(id: &str) -> Option<&'static dyn LicenseExt> {
    match id {
        "AGPL-3.0-only" => Some(&AGPL_3_0_only),
        "Apache-2.0" => Some(&Apache_2_0),
        "CC0-1.0" => Some(&CC0_1_0),
        "GPL-3.0-only" => Some(&GPL_3_0_only),
        "LGPL-3.0-only" => Some(&LGPL_3_0_only),
        "MIT" => Some(&MIT),
        "MPL-2.0" => Some(&MPL_2_0),
        "Unlicense" => Some(&Unlicense),
        _ => None,
    }
}

/// The permissions of the license.
#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Permissions {
    /// May be used for commercial purposes.
    pub commercial_use: bool,
    /// May be distributed.
    pub distribution: bool,
    /// May be modified.
    pub modification: bool,
    /// Provides an express grant of patent rights from contributors.
    pub patent_rights: bool,
    /// May be used for private purposes.
    pub private_use: bool,
}

impl Display for Permissions {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.commercial_use {
            f.write_str("- May be used for commercial purposes.\n")?;
        }
        if self.distribution {
            f.write_str("- May be distributed.\n")?;
        }
        if self.modification {
            f.write_str("- May be modified.\n")?;
        }
        if self.patent_rights {
            f.write_str("- Provides an express grant of patent rights from contributors.\n")?;
        }
        if self.private_use {
            f.write_str("- May be used for private purposes.\n")?;
        }
        Ok(())
    }
}

/// The conditions of the license.
#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Conditions {
    /// Source code must be made available when the software is distributed.
    pub disclose_sources: bool,
    /// Changes made to the code must be documented.
    pub document_changes: bool,
    /// The license and copyright notice must be included with the software.
    pub license_and_copyright_notice: bool,
    /// Users who interact with the software via network are
    /// given the right to receive a copy of the source code.
    pub network_use_is_distribution: bool,
    /// Modifications must be released under the same license.
    pub same_license: bool,
}

impl Display for Conditions {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.disclose_sources {
            f.write_str(
                "- Source code must be made available when the software is distributed.\n",
            )?;
        }
        if self.document_changes {
            f.write_str("- Changes made to the code must be documented.\n")?;
        }
        if self.license_and_copyright_notice {
            f.write_str(
                "- The license and copyright notice must be included with the software.\n",
            )?;
        }
        if self.network_use_is_distribution {
            f.write_str("- Users who interact with the software via network are given the right to receive a copy of the source code.\n")?;
        }
        if self.same_license {
            f.write_str("- Modifications must be released under the same license.\n")?;
        }
        Ok(())
    }
}

/// The limitations of the license.
#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Limitations {
    /// Includes a limitation of liability.
    pub no_liability: bool,
    /// Does not grant trademark rights.
    pub no_trademark_rights: bool,
    /// Does not provide any warranty.
    pub no_warranty: bool,
    /// Does not provide any rights in the patents of contributors.
    pub no_patent_rights: bool,
}

impl Display for Limitations {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.no_liability {
            f.write_str("- Includes a limitation of liability.\n")?;
        }
        if self.no_trademark_rights {
            f.write_str("- Does not grant trademark rights.\n")?;
        }
        if self.no_warranty {
            f.write_str("- Does not provide any warranty.\n")?;
        }
        if self.no_patent_rights {
            f.write_str("- Does not provide any rights in the patents of contributors.\n")?;
        }
        Ok(())
    }
}

/// Extension trait for supported licenses.
pub trait LicenseExt: License {
    /// The permissions of the license.
    fn permissions(&self) -> Permissions;

    /// The conditions of the license.
    fn conditions(&self) -> Conditions;

    /// The limitations of the license.
    fn limitations(&self) -> Limitations;
}

impl LicenseExt for AGPL_3_0_only {
    #[inline]
    fn permissions(&self) -> Permissions {
        Permissions {
            commercial_use: true,
            distribution: true,
            modification: true,
            patent_rights: true,
            private_use: true,
        }
    }

    #[inline]
    fn conditions(&self) -> Conditions {
        Conditions {
            disclose_sources: true,
            document_changes: true,
            license_and_copyright_notice: true,
            network_use_is_distribution: true,
            same_license: true,
        }
    }

    #[inline]
    fn limitations(&self) -> Limitations {
        Limitations {
            no_liability: true,
            no_trademark_rights: false,
            no_warranty: true,
            no_patent_rights: false,
        }
    }
}

impl LicenseExt for Apache_2_0 {
    #[inline]
    fn permissions(&self) -> Permissions {
        Permissions {
            commercial_use: true,
            distribution: true,
            modification: true,
            patent_rights: true,
            private_use: true,
        }
    }

    #[inline]
    fn conditions(&self) -> Conditions {
        Conditions {
            disclose_sources: false,
            document_changes: true,
            license_and_copyright_notice: true,
            network_use_is_distribution: false,
            same_license: false,
        }
    }

    #[inline]
    fn limitations(&self) -> Limitations {
        Limitations {
            no_liability: true,
            no_trademark_rights: true,
            no_warranty: true,
            no_patent_rights: false,
        }
    }
}

impl LicenseExt for CC0_1_0 {
    #[inline]
    fn permissions(&self) -> Permissions {
        Permissions {
            commercial_use: true,
            distribution: true,
            modification: true,
            patent_rights: false,
            private_use: true,
        }
    }

    #[inline]
    fn conditions(&self) -> Conditions {
        Conditions {
            disclose_sources: false,
            document_changes: false,
            license_and_copyright_notice: false,
            network_use_is_distribution: false,
            same_license: false,
        }
    }

    #[inline]
    fn limitations(&self) -> Limitations {
        Limitations {
            no_liability: true,
            no_trademark_rights: true,
            no_warranty: true,
            no_patent_rights: true,
        }
    }
}

impl LicenseExt for GPL_3_0_only {
    #[inline]
    fn permissions(&self) -> Permissions {
        Permissions {
            commercial_use: true,
            distribution: true,
            modification: true,
            patent_rights: true,
            private_use: true,
        }
    }

    #[inline]
    fn conditions(&self) -> Conditions {
        Conditions {
            disclose_sources: true,
            document_changes: true,
            license_and_copyright_notice: true,
            network_use_is_distribution: false,
            same_license: true,
        }
    }

    #[inline]
    fn limitations(&self) -> Limitations {
        Limitations {
            no_liability: true,
            no_trademark_rights: false,
            no_warranty: true,
            no_patent_rights: false,
        }
    }
}

impl LicenseExt for LGPL_3_0_only {
    #[inline]
    fn permissions(&self) -> Permissions {
        Permissions {
            commercial_use: true,
            distribution: true,
            modification: true,
            patent_rights: true,
            private_use: true,
        }
    }

    #[inline]
    fn conditions(&self) -> Conditions {
        Conditions {
            disclose_sources: true,
            document_changes: true,
            license_and_copyright_notice: true,
            network_use_is_distribution: false,
            same_license: true,
        }
    }

    #[inline]
    fn limitations(&self) -> Limitations {
        Limitations {
            no_liability: true,
            no_trademark_rights: false,
            no_warranty: true,
            no_patent_rights: false,
        }
    }
}

impl LicenseExt for MIT {
    #[inline]
    fn permissions(&self) -> Permissions {
        Permissions {
            commercial_use: true,
            distribution: true,
            modification: true,
            patent_rights: false,
            private_use: true,
        }
    }

    #[inline]
    fn conditions(&self) -> Conditions {
        Conditions {
            disclose_sources: false,
            document_changes: false,
            license_and_copyright_notice: true,
            network_use_is_distribution: false,
            same_license: false,
        }
    }

    #[inline]
    fn limitations(&self) -> Limitations {
        Limitations {
            no_liability: true,
            no_trademark_rights: false,
            no_warranty: true,
            no_patent_rights: false,
        }
    }
}

impl LicenseExt for MPL_2_0 {
    #[inline]
    fn permissions(&self) -> Permissions {
        Permissions {
            commercial_use: true,
            distribution: true,
            modification: true,
            patent_rights: true,
            private_use: true,
        }
    }

    #[inline]
    fn conditions(&self) -> Conditions {
        Conditions {
            disclose_sources: true,
            document_changes: false,
            license_and_copyright_notice: true,
            network_use_is_distribution: false,
            same_license: true,
        }
    }

    #[inline]
    fn limitations(&self) -> Limitations {
        Limitations {
            no_liability: true,
            no_trademark_rights: true,
            no_warranty: true,
            no_patent_rights: false,
        }
    }
}

impl LicenseExt for Unlicense {
    #[inline]
    fn permissions(&self) -> Permissions {
        Permissions {
            commercial_use: true,
            distribution: true,
            modification: true,
            patent_rights: false,
            private_use: true,
        }
    }

    #[inline]
    fn conditions(&self) -> Conditions {
        Conditions {
            disclose_sources: false,
            document_changes: false,
            license_and_copyright_notice: false,
            network_use_is_distribution: false,
            same_license: false,
        }
    }

    #[inline]
    fn limitations(&self) -> Limitations {
        Limitations {
            no_liability: true,
            no_trademark_rights: false,
            no_warranty: true,
            no_patent_rights: false,
        }
    }
}
