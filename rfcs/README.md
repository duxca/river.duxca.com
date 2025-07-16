# RFCs (Request for Comments)

This directory contains RFC documents for managing technical changes and feature requests in the river.duxca.com project. RFCs serve as structured work orders for Claude Code to implement changes systematically.

## Purpose

RFCs provide a formal process for:
- Proposing technical changes and new features
- Documenting implementation decisions
- Creating clear work instructions for Claude Code
- Maintaining a historical record of technical decisions
- Facilitating structured discussion and feedback

## File Naming Convention

All RFC files **MUST** follow this naming pattern:
```
YYYY-MM-DD-brief-description.md
```

### Examples:
- `2024-07-16-add-user-authentication.md`
- `2024-07-16-implement-oauth-login.md`
- `2024-07-16-refactor-map-components.md`

### Date Format Rules:
- Use ISO 8601 format: `YYYY-MM-DD`
- Year: 4 digits (e.g., 2024)
- Month: 2 digits with leading zero (e.g., 07)
- Day: 2 digits with leading zero (e.g., 16)
- This format ensures chronological sorting and global consistency

## RFC Document Structure

Each RFC document **MUST** be written in Markdown format and include the following sections:

### Required Sections:

```markdown
# RFC YYYY-MM-DD: Title

**Status**: [Draft/In Progress/Completed/Rejected]
**Author**: [Author Name]
**Created**: YYYY-MM-DD
**Updated**: YYYY-MM-DD

## Overview
Brief 1-2 paragraph summary of the proposed change.

## Background
Detailed context explaining why this change is needed.

## Detailed Design
Comprehensive technical specification of the implementation.

## Implementation Plan
Step-by-step breakdown of tasks for Claude Code.

## Testing Strategy
How to verify the implementation works correctly.

## Rollout Plan
Deployment and migration considerations.

## Alternatives Considered
Other approaches that were evaluated.

## Risks and Mitigation
Potential issues and how to handle them.

## Future Considerations
Long-term implications and follow-up work.
```

## Claude Code Work Order Guidelines

Since these RFCs serve as work orders for Claude Code, follow these best practices:

### 1. Implementation Plan Section
- Break down work into specific, actionable tasks
- Include file paths and function names when relevant
- Specify testing requirements
- List any dependencies or prerequisites

### 2. Technical Specifications
- Use precise technical language
- Include code examples where helpful
- Reference existing patterns in the codebase
- Specify expected behavior and edge cases

### 3. Context and Constraints
- Reference relevant files in the codebase
- Include any architectural constraints
- Mention performance requirements
- Note security considerations

### 4. Acceptance Criteria
- Define what "done" means
- Include specific test cases
- List required documentation updates
- Specify deployment requirements

## RFC Lifecycle

### 1. Draft Phase
- Create RFC document with all required sections
- Set status to "Draft"
- Conduct initial review and feedback

### 2. In Progress Phase
- Change status to "In Progress"
- Claude Code begins implementation
- Update document with progress notes

### 3. Completed Phase
- All implementation tasks finished
- Tests pass
- Documentation updated
- Status changed to "Completed"

### 4. Rejected Phase
- RFC deemed unnecessary or infeasible
- Status changed to "Rejected"
- Include rejection reason in document

## Best Practices

### For RFC Authors:
1. **Be Specific**: Vague requirements lead to poor implementations
2. **Include Examples**: Show expected inputs/outputs
3. **Reference Existing Code**: Point to similar patterns in the codebase
4. **Consider Edge Cases**: Think about error conditions and boundary cases
5. **Plan for Testing**: Specify how to verify the implementation

### For Claude Code Implementation:
1. **Follow the Implementation Plan**: Use the RFC as a checklist
2. **Update Status**: Change RFC status when beginning and completing work
3. **Document Changes**: Update the RFC with any deviations from the plan
4. **Test Thoroughly**: Ensure all acceptance criteria are met
5. **Update Related Documentation**: Keep README.md and other docs current

### File Organization:
- Keep RFCs focused on single features/changes
- Use descriptive but concise titles
- Maintain chronological order through date prefixes
- Archive old RFCs rather than deleting them

## Common RFC Types

### Feature RFCs
New functionality or capabilities

### Architecture RFCs
System design changes or improvements

### Process RFCs
Development workflow or deployment changes

### Bug Fix RFCs
Complex bug fixes requiring detailed planning

### Performance RFCs
Optimization or scalability improvements

## Review Process

1. **Self-Review**: Author reviews RFC for completeness
2. **Technical Review**: Evaluate technical feasibility
3. **Implementation Review**: Claude Code assesses work breakdown
4. **Approval**: RFC approved for implementation

## Tools and Resources

### Useful Commands:
```bash
# List all RFCs chronologically
ls -la rfcs/

# Search RFCs by content
grep -r "search-term" rfcs/

# Find RFCs by date range
ls rfcs/2024-07-* 
```

### Related Documentation:
- `/CLAUDE.md` - Project-specific Claude Code guidelines
- `/README.md` - Main project documentation
- `/db/README.md` - Database management guide

## Templates

### Quick RFC Template:
```markdown
# RFC YYYY-MM-DD: [Title]

**Status**: Draft  
**Author**: [Your Name]  
**Created**: YYYY-MM-DD  

## Overview
[Brief description]

## Implementation Plan
1. [ ] Task 1
2. [ ] Task 2
3. [ ] Task 3

## Testing
- [ ] Unit tests
- [ ] Integration tests
- [ ] Manual testing

## Acceptance Criteria
- [ ] Feature works as described
- [ ] Tests pass
- [ ] Documentation updated
```

## Maintenance

- Review and update this README as the RFC process evolves
- Archive completed RFCs older than 6 months to `rfcs/archive/`
- Maintain an index of major RFCs for quick reference
- Update templates based on lessons learned

---

*This RFC process is designed to scale with the project and provide clear guidance for Claude Code implementations. For questions or suggestions, update this README through the standard RFC process.*