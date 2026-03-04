enum VibratoParameter {
  INTENSITY = 'intensity',
}

export type VibratoSettings = {
  parameters: Record<VibratoParameter, number>,
}
