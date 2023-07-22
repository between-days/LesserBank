import { ThemeIcon, Tooltip, Image } from "@mantine/core"
import { ACCOUNT_ICON_SIZE, TRANSACTION_ICON_STATUS_SIZE } from "./UIConstants"
import { IconArrowsUp, IconCircleCheck, IconCloudDollar, IconExclamationCircle, IconReload, IconTimeDuration0 } from "@tabler/icons-react"
import { AccountType, TransactionStatus } from "./interfaces"

export const getPrettyDate = (date: string) => new Date(date).toLocaleDateString('en-au')

export const getPrettyDateTime = (date: string) => new Date(date).toLocaleTimeString('en-au')

export const getIconForTransactionStatus = (transactionStatus: TransactionStatus) => {
    switch (transactionStatus) {
        case "complete":
            return <Tooltip label="complete" position="right" transitionProps={{ duration: 0 }}>
                <ThemeIcon size={TRANSACTION_ICON_STATUS_SIZE} variant="light" radius="md">
                    <IconCircleCheck />
                </ThemeIcon>

            </Tooltip>
        case "pending":
            return <Tooltip label="pending" position="right" transitionProps={{ duration: 0 }}>
                <ThemeIcon size={TRANSACTION_ICON_STATUS_SIZE} variant="light" radius="md">
                    <IconReload />
                </ThemeIcon>

            </Tooltip>
        default:
            return <Tooltip label={"error"} position="right" transitionProps={{ duration: 0 }}>
                <ThemeIcon size={TRANSACTION_ICON_STATUS_SIZE} variant="outline" radius="md">
                    <IconExclamationCircle />
                </ThemeIcon>
            </Tooltip>
    }
}






export function getIconForAccountType(accountType: AccountType) {
    switch (accountType) {
        case "savings":
            return <Tooltip label="Savings" position="right" transitionProps={{ duration: 0 }}>
                <ThemeIcon size={ACCOUNT_ICON_SIZE} variant="light" radius="md">
                    <IconCloudDollar />
                </ThemeIcon>

            </Tooltip>
        case "termDeposit":
            return <Tooltip label="Term Deposit" position="right" transitionProps={{ duration: 0 }}>
                <ThemeIcon size={ACCOUNT_ICON_SIZE} variant="light" radius="md">
                    <IconTimeDuration0 />
                </ThemeIcon>

            </Tooltip>
        case "transaction":
            return <Tooltip label="Transaction" position="right" transitionProps={{ duration: 0 }}>
                <ThemeIcon size={ACCOUNT_ICON_SIZE} variant="light" radius="md">
                    <IconArrowsUp />
                </ThemeIcon>

            </Tooltip>
        default:
            return <Tooltip label={"Unkown Type"} position="right" transitionProps={{ duration: 0 }}>
                <ThemeIcon size={ACCOUNT_ICON_SIZE} variant="outline" radius="md">
                    <IconCloudDollar />
                </ThemeIcon>
            </Tooltip>
    }
}

export function getDollarTextFromCents(balanceCents: number) {
    const balanceDollars = balanceCents / 100
    return `$${balanceDollars.toLocaleString()}`
}

export function getBsbString(bsb: number) {
    const bsbString = bsb.toString()
    return bsbString.substring(0, 3) + " " + bsbString.substring(3, 6)
}

export function getAccountNumberString(accountNumber: number) {
    const anString = accountNumber.toString()
    return anString.substring(0, 3) + " " + anString.substring(3, 6) + " " + anString.substring(6, 10)
}

export function getAccountCardImage(accountType: AccountType) {
    switch (accountType) {
        case "savings":
            return <Image
                src="https://images.unsplash.com/photo-1579621970588-a35d0e7ab9b6?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=3540&q=80"
                height={150}
                alt="Savings"
            />
        case "termDeposit":
            return <Image
                src="https://images.unsplash.com/photo-1531161348856-92e5c7b2e6de?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=3542&q=80"
                height={150}
                alt="Term Deposit"
            />
        case "transaction":
            return <Image
                src="https://images.unsplash.com/photo-1488330890490-c291ecf62571?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=2370&q=80"
                height={150}
                alt="Transaction"
            />
        default:
            return <Image
                src="https://images.unsplash.com/photo-1579621970588-a35d0e7ab9b6?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=3540&q=80"
                height={150}
                alt="Unknown"
            />
    }
}