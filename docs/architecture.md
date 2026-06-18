# Architecture Reference

See [ARCHITECTURE.md](../ARCHITECTURE.md) for the full architecture overview including data flow diagram and crate responsibilities.

## Graph API Endpoints Used

| Endpoint | Purpose |
|---|---|
| `GET /v1.0/users` | List all tenant users |
| `GET /v1.0/roleManagement/directory/roleAssignments` | Active role assignments |
| `GET /v1.0/roleManagement/directory/roleEligibilitySchedules` | PIM eligible assignments |
| `GET /v1.0/roleManagement/directory/roleAssignmentSchedules` | PIM active assignments |
| `GET /v1.0/policies/roleManagementPolicyAssignments` | PIM role settings |

All requests are read-only GET requests. The tool never issues POST, PATCH, PUT, or DELETE requests.
