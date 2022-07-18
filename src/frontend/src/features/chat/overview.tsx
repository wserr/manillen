
interface OverviewProps {
    Messages: string[]
}
export const Overview = (props: OverviewProps) =>
    <ul>
        { props.Messages.map(m => <li>{m}</li>)}
    </ul>
