enum AmplifierParameter {
  GAIN = 'gain',
}

export type AmplifierSettings = {
  parameters: Record<AmplifierParameter, number>,
}
