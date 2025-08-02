export type FetchFn = (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>;

export type ContributionsCalendar = {
	totalContributions: number;
	contributions: {
		date: string;
		contributionCount: number;
		color: string;
	}[];
};

export class GitHub {
	private readonly fetch;
	private readonly token;

	constructor(fetch: FetchFn, token: string) {
		this.fetch = fetch;
		this.token = token;
	}

	public async getContributionsCalendar(username: string): Promise<ContributionsCalendar> {
		const headers = {
			Authorization: `bearer ${this.token}`
		};
		const body = {
			query: `query {
              user(login: "${username}") {
                name
                contributionsCollection {
                  contributionCalendar {
                    colors
                    totalContributions
                    weeks {
                      contributionDays {
                        color
                        contributionCount
                        date
                        weekday
                      }
                      firstDay
                    }
                  }
                }
              }
            }`
		};

		const response = await this.fetch('https://api.github.com/graphql', {
			method: 'POST',
			body: JSON.stringify(body),
			headers: headers
		});

		if (response.ok) {
			const { data } = await response.json();

			if (data.user.contributionsCollection) {
				const { totalContributions, weeks } =
					data.user.contributionsCollection.contributionCalendar;
				const contributions = weeks.flatMap(
					(week: {
						contributionDays: { date: string; contributionCount: number; color: string }[];
					}) =>
						week.contributionDays.map(({ date, contributionCount, color }) => ({
							date,
							contributionCount,
							color
						}))
				);

				return {
					totalContributions,
					contributions
				};
			}
		}

		throw new Error('No contributions found');
	}
}
