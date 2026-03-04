enum ChorusParameter {
  DEPTH = 'depth',
  MIX = 'mix',
}

export type ChorusSettings = {
  parameters: Record<ChorusParameter, number>,
}
