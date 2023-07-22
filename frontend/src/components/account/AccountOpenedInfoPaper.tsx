import { getPrettyDate, getPrettyDateTime } from "@/UIUtils"
import { Group, Paper, Stack, Text, ThemeIcon, Tooltip } from "@mantine/core"
import { IconCalendar } from "@tabler/icons-react"

export const AccountOpenedInfoPaper = ({ dateOpened }: { dateOpened: string }) => {
    return <Paper withBorder shadow="sm" radius="lg" p="md" key={"aaaaa"} h="5rem">
        <Group>
            <Tooltip label={getPrettyDateTime(dateOpened)}>
                <ThemeIcon size="xl" variant="light" radius="md">
                    <IconCalendar />
                </ThemeIcon>
            </Tooltip>
            <Stack spacing={0}>
                <Text fz="md" fw={500}>
                    {getPrettyDate(dateOpened)}
                </Text>
                <Text color="dimmed" fz="xs">
                    Opened
                </Text>
            </Stack>
        </Group>
    </Paper>

}