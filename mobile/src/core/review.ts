import ClearSignCore from '../../modules/clearsign-core/src/ClearSignCoreModule';

export type ReviewSeverity = 'clear' | 'warning' | 'blind' | 'critical' | 'refused';

export type ReviewFinding = {
  number: number;
  severity: string;
  code: string;
  message: string;
};

export type ReviewSection = {
  title: string;
  fields: [string, string][];
};

export type ReviewResponse = {
  apiVersion: number;
  ok: boolean;
  severity: ReviewSeverity;
  message?: string;
  hash?: string;
  hashConfirmed?: boolean;
  domain?: string;
  domainSource?: string;
  chainIdSource?: string;
  chainBound?: boolean;
  sections?: ReviewSection[];
  findings?: ReviewFinding[];
  mustAcknowledge?: { number: number; code: string }[];
  text?: string;
};

const SUPPORTED_SEVERITIES = new Set<ReviewSeverity>([
  'clear',
  'warning',
  'blind',
  'critical',
  'refused',
]);

export const engineAvailable = ClearSignCore !== null;

export async function reviewSafeJson(
  jsonText: string,
  chainId: string,
  version: number,
): Promise<ReviewResponse> {
  if (!/^[0-9]+$/.test(chainId) || decimalExceedsU64(chainId)) {
    throw new Error('Chain ID must be a whole number from 0 through 18446744073709551615.');
  }
  if (version !== 0 && version !== 1 && version !== 3) {
    throw new Error('Safe domain must be automatic, v1.1.x or v1.3.0 and later.');
  }
  if (!ClearSignCore) {
    throw new Error(
      'The native ClearSign engine is not in this build. Use a ClearSign development or release build, not Expo Go.',
    );
  }
  if (ClearSignCore.apiVersion !== 1) {
    throw new Error(`Unsupported ClearSign native API version: ${ClearSignCore.apiVersion}.`);
  }

  const text = await ClearSignCore.reviewSafeJson(jsonText, chainId, version);
  const value: unknown = JSON.parse(text);
  if (!isReviewResponse(value)) {
    throw new Error('The native ClearSign engine returned a response this app cannot verify.');
  }
  return value;
}

function decimalExceedsU64(value: string): boolean {
  const normalized = value.replace(/^0+(?=[0-9])/, '');
  const maximum = '18446744073709551615';
  return normalized.length > maximum.length ||
    (normalized.length === maximum.length && normalized > maximum);
}

function isReviewResponse(value: unknown): value is ReviewResponse {
  if (!value || typeof value !== 'object') return false;
  const response = value as Partial<ReviewResponse>;
  const findingsValid = response.findings === undefined || (
    Array.isArray(response.findings) && response.findings.every((finding) =>
      Number.isSafeInteger(finding.number) && finding.number > 0 &&
      typeof finding.severity === 'string' && typeof finding.code === 'string' &&
      typeof finding.message === 'string') &&
    response.findings.every((finding, index) => finding.number === index + 1)
  );
  const acknowledgementsValid = response.mustAcknowledge === undefined || (
    Array.isArray(response.mustAcknowledge) && response.mustAcknowledge.every((required) =>
      Number.isSafeInteger(required.number) && typeof required.code === 'string' &&
      response.findings?.some((finding) =>
        finding.number === required.number && finding.code === required.code))
  );
  return (
    response.apiVersion === 1 &&
    typeof response.ok === 'boolean' &&
    typeof response.severity === 'string' &&
    SUPPORTED_SEVERITIES.has(response.severity as ReviewSeverity) &&
    findingsValid && acknowledgementsValid
  );
}
