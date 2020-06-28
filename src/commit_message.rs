use regex::Regex;
/// A `CommitMessage`, the primary entry point to the library

    /// Does the commit match the saved portions of the commit
    ///
    /// This takes a regex and matches it to the visible portions of the commits, so it excludes
    /// comments, and everything after the scissors.
    ///
    /// # Examples
    ///
    /// ```
    /// use indoc::indoc;
    /// use mit_commit::CommitMessage;
    /// use regex::Regex;
    ///
    /// let commit = CommitMessage::from(indoc!(
    ///     "
    ///        Example Commit Message
    ///
    ///        This is an example commit message for linting
    ///        ; Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
    ///        ; die mit ';' beginnen, werden ignoriert, und eine leere Beschreibung
    ///        ; bricht den Commit ab.
    ///        ;
    ///        ; Auf Branch main
    ///        ; Ihr Branch ist auf demselben Stand wie 'origin/main'.
    ///        ;
    ///        ; Zum Commit vorgemerkte \u{00E4}nderungen:
    ///        ;    neue Datei:     file
    ///        ;
    ///        "
    /// ));
    ///
    /// let re = Regex::new("[Bb]itte").unwrap();
    /// assert_eq!(commit.matches_pattern(&re), false);
    ///
    /// let re = Regex::new("f[o\u{00FC}]r linting").unwrap();
    /// assert_eq!(commit.matches_pattern(&re), true);
    ///
    /// let re = Regex::new("[Ee]xample Commit Message").unwrap();
    /// assert_eq!(commit.matches_pattern(&re), true);
    /// ```
    pub fn matches_pattern(&self, re: &Regex) -> bool {
        let subject = self.clone().get_subject();
        let bodies = self.clone().get_body();

        let commit_text = format!("{}{}", subject, bodies);
        re.is_match(&commit_text)
    }
    use regex::Regex;

    #[test]
    fn can_check_if_it_matches_pattern() {
        let commit = CommitMessage::from(indoc!(
            "
            Example Commit Message

            This is an example commit message for linting
            # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
            # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
            # bricht den Commit ab.
            #
            # Auf Branch main
            # Ihr Branch ist auf demselben Stand wie 'origin/main'.
            #
            # Zum Commit vorgemerkte \u{00E4}nderungen:
            #	neue Datei:     file
            #
            "
        ));

        let re = Regex::new("[Bb]itte").unwrap();
        assert_eq!(commit.matches_pattern(&re), false);

        let re = Regex::new("f[o\u{00FC}]r linting").unwrap();
        assert_eq!(commit.matches_pattern(&re), true);

        let re = Regex::new("[Ee]xample Commit Message").unwrap();
        assert_eq!(commit.matches_pattern(&re), true);
    }