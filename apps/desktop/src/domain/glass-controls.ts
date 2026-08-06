export interface GlassSelectOption {
  value: string;
  label: string;
  description?: string;
  disabled?: boolean;
}

function enabledIndexes(
  options: readonly GlassSelectOption[],
): number[] {
  return options.flatMap((option, index) =>
    option.disabled ? [] : [index],
  );
}

export function firstEnabledOptionIndex(
  options: readonly GlassSelectOption[],
): number {
  return enabledIndexes(options)[0] ?? -1;
}

export function selectedEnabledOptionIndex(
  options: readonly GlassSelectOption[],
  value: string,
): number {
  const selectedIndex = options.findIndex(
    (option) => option.value === value && !option.disabled,
  );

  return selectedIndex >= 0
    ? selectedIndex
    : firstEnabledOptionIndex(options);
}

export function moveEnabledOptionIndex(
  options: readonly GlassSelectOption[],
  currentIndex: number,
  direction: -1 | 1,
): number {
  const indexes = enabledIndexes(options);

  if (!indexes.length) return -1;

  const currentPosition = indexes.indexOf(currentIndex);

  if (currentPosition < 0) {
    return direction === 1
      ? indexes[0] ?? -1
      : indexes.at(-1) ?? -1;
  }

  const nextPosition =
    (currentPosition + direction + indexes.length) % indexes.length;

  return indexes[nextPosition] ?? -1;
}

export function edgeEnabledOptionIndex(
  options: readonly GlassSelectOption[],
  edge: 'first' | 'last',
): number {
  const indexes = enabledIndexes(options);

  if (!indexes.length) return -1;

  return edge === 'first'
    ? indexes[0] ?? -1
    : indexes.at(-1) ?? -1;
}
