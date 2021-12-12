# Development strategy

1. Divide the job in two independent steps:
   1. **Transactions:** implement two-phase-commit between AlGlobo and services.
   2. ~~**Replicas:** implement critical-mission service, syncing state.~~
2. Once both are completed, integrate to get final job.
3. Add-ons: tests, UI, etc.
