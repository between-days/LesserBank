import { MantineSize, Stack, Text } from "@mantine/core"

interface TextAndSubtextProps {
    text: string
    subText: string
    textSize?: MantineSize
    subTextSize?: MantineSize
}

export const TextAndSubtext = ({ text, subText, textSize, subTextSize }: TextAndSubtextProps) => {
    textSize = textSize ?? 'lg'
    subTextSize = subTextSize ?? 'md'

    return <Stack spacing={0}>
        <Text fz={textSize} fw={500}>
            {text}
        </Text>
        <Text fz={subTextSize} color="dimmed">
            {subText}
        </Text>
    </Stack>
}