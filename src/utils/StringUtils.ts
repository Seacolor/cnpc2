export function format(label: string, ...values: string[]): string {
  for (const [i, arg] of values.entries()) {
    const regExp = new RegExp(`\\{${i}\\}`, 'g')
    label = label.replace(regExp, arg as string)
  }
  return label
}

export function arrayEqual(a: string[], b: string[]): boolean {
  if (!Array.isArray(a))    return false;
  if (!Array.isArray(b))    return false;
  if (a.length != b.length) return false;
  for (var i = 0, n = a.length; i < n; ++i) {
    if (a[i] !== b[i]) return false;
  }
  return true;
}
