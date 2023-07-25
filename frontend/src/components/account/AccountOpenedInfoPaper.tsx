import { getPrettyDate, getPrettyDateTime } from "@/UIUtils"
import { Group, Paper, ThemeIcon, Tooltip } from "@mantine/core"
import { IconCalendar } from "@tabler/icons-react"
import { TextAndSubtext } from "../shared/TextAndSubtext"

export const AccountOpenedInfoPaper = ({ dateOpened }: { dateOpened: string }) => {
    return <Paper withBorder shadow="sm" radius="lg" p="md">
        <Group>
            <Tooltip label={getPrettyDateTime(dateOpened)}>
                <ThemeIcon size="xl" variant="light" radius="md">
                    <IconCalendar />
                </ThemeIcon>
            </Tooltip>
            <TextAndSubtext text={getPrettyDate(dateOpened)} subText="Opened" textSize="md" subTextSize="xs" />
        </Group>
    </Paper>
}