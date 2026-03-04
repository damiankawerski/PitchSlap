enum PitchShifterParameter {
  SHIFT = 'shift',
}

export type PitchShifterSettings = {
  parameters: Record<PitchShifterParameter, number>,
}
