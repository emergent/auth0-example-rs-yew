pub struct ContentData {
    pub title: &'static str,
    pub link: &'static str,
    pub description: &'static str,
}

pub const CONTENT_DATA: &[ContentData] = &[
    ContentData{
    title: "Configure other identity providers",
    link: "https://auth0.com/docs/connections",
    description:
      "Auth0 supports social providers as Facebook, Twitter, Instagram and 100+, Enterprise providers as Microsoft Office 365, Google Apps, Azure, and more. You can also use any OAuth2 Authorization Server."
  },
  ContentData{
    title: "Enable Multifactor Authentication",
    link: "https://auth0.com/docs/multifactor-authentication",
    description:
      "Add an extra layer of security by enabling Multi-factor Authentication, requiring your users to provide more than one piece of identifying information. Push notifications, authenticator apps, SMS, and DUO Security are supported."
  },
  ContentData{
    title: "Anomaly Detection",
    link: "https://auth0.com/docs/anomaly-detection",
    description:
      "Auth0 can detect anomalies and stop malicious attempts to access your application. Anomaly detection can alert you and your users of suspicious activity, as well as block further login attempts."
  },
  ContentData{
    title: "Learn About Rules",
    link: "https://auth0.com/docs/rules",
    description:
      "Rules are JavaScript functions that execute when a user authenticates to your application. They run once the authentication process is complete, and you can use them to customize and extend Auth0's capabilities."
  }
];
