export type WindowMode = 'main' | 'widget';

export function resolveWindowMode(search: string): WindowMode {
  const params = new URLSearchParams(search);
  return params.get('window') === 'widget' ? 'widget' : 'main';
}
