# Services

This directory contains infrastructure boundaries such as settings, filesystem,
network, database, and operating-system integrations.

A service should not render UI or directly own page state. Return typed data and
contextual errors; the calling GPUI entity decides how results change state and
which notice to show. Keep trait abstractions demand-driven—introduce one when
there are multiple implementations or tests need a boundary, not preemptively.

`settings.rs` demonstrates the pattern: serialization and crash-resistant file
replacement live here, while `RootView` owns the current preferences and visible
error policy.
