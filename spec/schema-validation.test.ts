/**
 * RWP Schema Validation Tests
 * Validates all 5 RWP schemas against real artifact examples
 *
 * Tests ensure:
 * - Schemas are well-formed JSON Schema draft-7
 * - Real artifacts validate successfully
 * - Edge cases are handled
 * - Error messages are clear
 */

import Ajv, { ValidateFunction } from 'ajv';
import planSchema from './schemas/plan.schema.json';
import intakeSchema from './schemas/intake.schema.json';
import manifestSchema from './schemas/manifest.schema.json';
import stateSchema from './schemas/state.schema.json';
import handoffSchema from './schemas/handoff.schema.json';

// Initialize validator
const ajv = new Ajv({ strict: false });

describe('RWP Schema Validation', () => {
  describe('Plan Schema', () => {
    const validatePlan: ValidateFunction = ajv.compile(planSchema);

    it('should validate a minimal valid plan', () => {
      const minimalPlan = {
        plan_id: 'MP-0001-quick-task',
        title: 'Quick Task',
        overview: 'A quick infrastructure task',
        created_at: '2026-03-04T10:00:00Z',
        phases: [
          {
            phase_id: 'P-01',
            title: 'Execute Task',
            objective: 'Complete the task',
            deliverables: ['Task completed'],
            tasks: ['Do the thing'],
            verification: ['Verify completion']
          }
        ]
      };

      const valid = validatePlan(minimalPlan);
      expect(valid).toBe(true);
      if (!valid) console.error(validatePlan.errors);
    });

    it('should validate a comprehensive plan with all fields', () => {
      const comprehensivePlan = {
        plan_id: 'MP-0002-infrastructure-modernization',
        title: 'Q1 Infrastructure Modernization',
        overview: 'Comprehensive modernization of our infrastructure stack',
        created_at: '2026-01-15T09:00:00Z',
        updated_at: '2026-03-04T15:30:00Z',
        owner: 'platform-team',
        classification: 'confidential',
        goals_and_success_criteria: {
          goals: [
            'Reduce deployment time by 50%',
            'Improve reliability to 99.99%'
          ],
          success_criteria: [
            'Deploy time < 5 minutes',
            'Uptime > 99.99% SLO'
          ]
        },
        phases: [
          {
            phase_id: 'P-01',
            title: 'Assessment Phase',
            objective: 'Understand current state and identify bottlenecks',
            duration_minutes: 480,
            deliverables: [
              'Architecture documentation',
              'Bottleneck analysis report'
            ],
            tasks: [
              'Map current deployment process',
              'Identify critical path items',
              'Document infrastructure topology'
            ],
            verification: [
              'Documentation reviewed by team',
              'Presentation delivered to stakeholders'
            ],
            dependencies: [],
            risks: [
              'Team might lack deep infrastructure knowledge'
            ]
          },
          {
            phase_id: 'P-02',
            title: 'Implementation Phase',
            objective: 'Implement modernization improvements',
            duration_minutes: 1440,
            deliverables: [
              'New deployment pipeline',
              'Infrastructure-as-Code'
            ],
            tasks: [
              'Set up new deployment infrastructure',
              'Migrate services to new platform'
            ],
            verification: [
              'All services deployed successfully',
              'Performance metrics meet targets'
            ],
            dependencies: ['P-01'],
            risks: [
              'Potential downtime during migration'
            ]
          }
        ]
      };

      const valid = validatePlan(comprehensivePlan);
      expect(valid).toBe(true);
      if (!valid) console.error(validatePlan.errors);
    });

    it('should validate plan with custom fields', () => {
      const planWithCustomFields = {
        plan_id: 'MP-0003-infrastructure-upgrade',
        title: 'Infrastructure Upgrade',
        overview: 'Modernize deployment pipeline',
        created_at: '2026-01-15T09:00:00Z',
        phases: [
          { phase_id: 'P-01', title: 'Upgrade', objective: 'obj', deliverables: [], tasks: [], verification: [] }
        ],
        custom_fields: {
          'x-billing-cost-center': 'ENG-2026-Q1',
          'x-billing-estimated-cost-usd': 45000,
          'x-security-requires-review': true,
          'x-security-compliance-frameworks': ['sox2', 'hipaa']
        }
      };

      const valid = validatePlan(planWithCustomFields);
      expect(valid).toBe(true);
      if (!valid) console.error(validatePlan.errors);
    });

    it('should reject plan missing required fields', () => {
      const invalidPlan = {
        plan_id: 'MP-0004-missing-title',
        // missing 'title'
        overview: 'A plan without a title',
        created_at: '2026-03-04T10:00:00Z',
        phases: []
      };

      const valid = validatePlan(invalidPlan);
      expect(valid).toBe(false);
      expect(validatePlan.errors).toBeDefined();
      expect(validatePlan.errors?.[0].message).toContain('title');
    });

    it('should validate phase_id pattern', () => {
      const planWithValidPhaseIds = {
        plan_id: 'MP-0005-valid-phase-ids',
        title: 'Test',
        overview: 'Test plan',
        created_at: '2026-03-04T10:00:00Z',
        phases: [
          { phase_id: 'P-01', title: 'Phase 1', objective: 'obj', deliverables: [], tasks: [], verification: [] },
          { phase_id: 'P-02-A', title: 'Phase 2A', objective: 'obj', deliverables: [], tasks: [], verification: [] },
          { phase_id: 'P-02-B', title: 'Phase 2B', objective: 'obj', deliverables: [], tasks: [], verification: [] },
          { phase_id: 'P-02-Z', title: 'Phase 2Z', objective: 'obj', deliverables: [], tasks: [], verification: [] }
        ]
      };

      const valid = validatePlan(planWithValidPhaseIds);
      expect(valid).toBe(true);

      const planWithInvalidPhaseId = {
        plan_id: 'MP-0006-invalid-phase-id',
        title: 'Test',
        overview: 'Test plan',
        created_at: '2026-03-04T10:00:00Z',
        phases: [
          { phase_id: 'INVALID-ID', title: 'Bad Phase', objective: 'obj', deliverables: [], tasks: [], verification: [] }
        ]
      };

      const valid2 = validatePlan(planWithInvalidPhaseId);
      expect(valid2).toBe(false);
    });
  });

  describe('Intake Schema', () => {
    const validateIntake: ValidateFunction = ajv.compile(intakeSchema);

    it('should validate a minimal valid intake', () => {
      const minimalIntake = {
        id: 'INT-0001',
        title: 'API Performance Issue',
        captured: '2026-03-04T12:00:00Z',
        pain_points: [
          {
            id: 'PP-001',
            description: 'P99 latency exceeds 1 second',
            impact: 'User-facing degradation'
          }
        ],
        requirements: [
          {
            id: 'REQ-001',
            description: 'Reduce P99 latency to < 200ms'
          }
        ],
        constraints: [
          'Cannot require database migration'
        ],
        success_criteria: [
          'P99 latency < 200ms measured over 24 hours'
        ]
      };

      const valid = validateIntake(minimalIntake);
      expect(valid).toBe(true);
      if (!valid) console.error(validateIntake.errors);
    });

    it('should validate intake with approval tracking', () => {
      const approvedIntake = {
        id: 'INT-0042',
        title: 'Security Audit Gap Remediation',
        summary: 'Address findings from Q1 2026 SOX2 audit',
        captured: '2026-02-28T14:30:00Z',
        approved_by: 'security-team',
        approval_date: '2026-03-01T09:00:00Z',
        classification: 'confidential',
        pain_points: [
          {
            id: 'PP-001',
            description: 'Insufficient encryption at rest',
            impact: 'Audit finding, non-compliant'
          }
        ],
        requirements: [
          {
            id: 'REQ-001',
            description: 'Enable encryption at rest for all databases'
          }
        ],
        constraints: [
          'Zero downtime required'
        ],
        success_criteria: [
          'All databases encrypted at rest',
          'Audit verified compliance'
        ]
      };

      const valid = validateIntake(approvedIntake);
      expect(valid).toBe(true);
      if (!valid) console.error(validateIntake.errors);
    });

    it('should reject intake missing required fields', () => {
      const invalidIntake = {
        id: 'INT-0001',
        // missing title
        captured: '2026-03-04T12:00:00Z',
        pain_points: [],
        requirements: [],
        constraints: [],
        success_criteria: []
      };

      const valid = validateIntake(invalidIntake);
      expect(valid).toBe(false);
    });

    it('should validate intake ID pattern', () => {
      const validIds = ['INT-0001', 'INT-9999'];
      const invalidIds = ['INT-001', 'INTAKE-001', 'int-0001'];

      validIds.forEach(id => {
        const intake = {
          id,
          title: 'Test Intake',
          captured: '2026-03-04T12:00:00Z',
          pain_points: [],
          requirements: [],
          constraints: [],
          success_criteria: []
        };
        const valid = validateIntake(intake);
        expect(valid).toBe(true);
      });

      invalidIds.forEach(id => {
        const intake = {
          id,
          title: 'Test Intake',
          captured: '2026-03-04T12:00:00Z',
          pain_points: [],
          requirements: [],
          constraints: [],
          success_criteria: []
        };
        const valid = validateIntake(intake);
        expect(valid).toBe(false);
      });
    });
  });

  describe('Manifest Schema', () => {
    const validateManifest: ValidateFunction = ajv.compile(manifestSchema);

    it('should validate a minimal manifest', () => {
      const minimalManifest = {
        id: 'MAN-0001',
        name: 'Q1 2026 Initiative',
        created_at: '2026-01-01T00:00:00Z',
        artifacts: [
          {
            artifact_id: 'INT-0001',
            artifact_type: 'intake'
          }
        ]
      };

      const valid = validateManifest(minimalManifest);
      expect(valid).toBe(true);
      if (!valid) console.error(validateManifest.errors);
    });

    it('should validate manifest with metadata', () => {
      const fullManifest = {
        id: 'MAN-2026-PLATFORM',
        name: 'Platform Modernization Manifest',
        description: 'Complete list of artifacts for Q1 platform work',
        version: '0.31.0',
        created_at: '2026-01-01T00:00:00Z',
        updated_at: '2026-03-04T15:30:00Z',
        artifacts: [
          {
            artifact_id: 'INT-0001',
            artifact_type: 'intake',
            title: 'Performance Issues'
          },
          {
            artifact_id: 'MP-0001-q1-infrastructure',
            artifact_type: 'plan',
            title: 'Q1 Infrastructure Plan'
          }
        ]
      };

      const valid = validateManifest(fullManifest);
      expect(valid).toBe(true);
      if (!valid) console.error(validateManifest.errors);
    });
  });

  describe('State Schema', () => {
    const validateState: ValidateFunction = ajv.compile(stateSchema);

    it('should validate a minimal state', () => {
      const minimalState = {
        plan_id: 'MP-0001-q1-infrastructure',
        execution: {
          status: 'in_progress',
          current_phase: 'P-01'
        },
        phases: {
          'P-01': {
            status: 'in_progress'
          }
        }
      };

      const valid = validateState(minimalState);
      expect(valid).toBe(true);
      if (!valid) console.error(validateState.errors);
    });

    it('should validate state with detailed tracking', () => {
      const detailedState = {
        plan_id: 'MP-0001-q1-infrastructure',
        request_id: 'INT-0001',
        title: 'Detailed state tracking',
        rwp_version: '0.31.0',
        execution: {
          status: 'paused',
          current_phase: 'P-02-Z',
          started_at: '2026-03-04T02:15:00Z',
          completed_at: null,
          last_heartbeat: '2026-03-04T04:45:00Z',
          heartbeat_timeout_minutes: 30
        },
        phases: {
          'P-01-A': {
            status: 'completed',
            started_at: '2026-03-04T02:15:00Z',
            completed_at: '2026-03-04T02:45:00Z'
          },
          'P-02-Z': {
            status: 'failed',
            started_at: '2026-03-04T04:00:00Z',
            completed_at: null
          }
        }
      };

      const valid = validateState(detailedState);
      expect(valid).toBe(true);
      if (!valid) console.error(validateState.errors);
    });
  });

  describe('Handoff Schema', () => {
    const validateHandoff: ValidateFunction = ajv.compile(handoffSchema);

    it('should validate a minimal handoff', () => {
      const minimalHandoff = {
        id: 'HO-MP-0235-P-02-A-2026-03-04',
        from_phase: 'P-02-A',
        to_phase: 'P-02-B',
        created_at: '2026-03-04T04:30:00Z',
        context_summary: 'UUID specification complete, reference implementations done'
      };

      const valid = validateHandoff(minimalHandoff);
      expect(valid).toBe(true);
      if (!valid) console.error(validateHandoff.errors);
    });

    it('should validate handoff with lessons and recommendations', () => {
      const fullHandoff = {
        id: 'HO-MP-0235-P-02-A-2026-03-04',
        from_phase: 'P-02-A',
        to_phase: 'P-02-B',
        created_at: '2026-03-04T04:30:00Z',
        context_summary: 'Completed UUID generation and sequence parser specifications',
        summary: 'All deliverables completed with high quality',
        lessons_learned: [
          'Automated validation catches issues early',
          'Custom fields need clear documentation'
        ],
        verified_by: 'qa-team'
      };

      const valid = validateHandoff(fullHandoff);
      expect(valid).toBe(true);
      if (!valid) console.error(validateHandoff.errors);
    });
  });

  describe('Schema Cross-Validation', () => {
    it('should validate all schemas are well-formed', () => {
      const schemas = [planSchema, intakeSchema, manifestSchema, stateSchema, handoffSchema];

      schemas.forEach(schema => {
        expect(schema.$schema).toBe('http://json-schema.org/draft-07/schema#');
        expect(schema.type).toBe('object');
        expect(schema.title).toBeDefined();
        expect(schema.description).toBeDefined();
      });
    });

    it('should compile all schemas without errors', () => {
      const schemas = {
        plan: planSchema,
        intake: intakeSchema,
        manifest: manifestSchema,
        state: stateSchema,
        handoff: handoffSchema
      };

      Object.entries(schemas).forEach(([name, schema]) => {
        const validator = ajv.compile(schema);
        expect(validator).toBeDefined();
        expect(typeof validator).toBe('function');
      });
    });
  });
});
