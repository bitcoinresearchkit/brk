const localDomains = new Set([
  "localhost",
  "127.0.0.1",
  "0.0.0.0",
  "::1",
  "[::1]",
]);

/**
 * @param {string} domain
 */
function isPrivateIpv4(domain) {
  const parts = domain.split(".").map(Number);

  if (
    parts.length !== 4 ||
    parts.some((part) => !Number.isInteger(part) || part < 0 || part > 255)
  ) {
    return false;
  }

  const [a, b] = parts;

  return (
    a === 10 ||
    a === 127 ||
    (a === 169 && b === 254) ||
    (a === 172 && b >= 16 && b <= 31) ||
    (a === 192 && b === 168)
  );
}

/**
 * @param {{ domain: string }} client
 */
export function isLocalClient(client) {
  const domain = client.domain.toLowerCase();

  return (
    localDomains.has(domain) ||
    domain.endsWith(".local") ||
    isPrivateIpv4(domain)
  );
}
