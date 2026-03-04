enum BitCrusherParameter {
  BIT_DEPTH = 'bit_depth',
}

export type BitCrusherSettings = {
  parameters: Record<BitCrusherParameter, number>,
}
