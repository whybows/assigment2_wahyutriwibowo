<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Post_users</name>
   <tag></tag>
   <elementGuidId>d1638cf6-582f-42d2-928a-ebff0c0e359d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;id\&quot;: 2,\n    \&quot;name\&quot;: \&quot;Ervin Howell\&quot;,\n    \&quot;username\&quot;: \&quot;Antonette\&quot;,\n    \&quot;email\&quot;: \&quot;Shanna@melissa.tv\&quot;,\n    \&quot;address\&quot;: {\n      \&quot;street\&quot;: \&quot;Victor Plains\&quot;,\n      \&quot;suite\&quot;: \&quot;Suite 879\&quot;,\n      \&quot;city\&quot;: \&quot;Wisokyburgh\&quot;,\n      \&quot;zipcode\&quot;: \&quot;90566-7771\&quot;,\n      \&quot;geo\&quot;: {\n        \&quot;lat\&quot;: \&quot;-43.9509\&quot;,\n        \&quot;lng\&quot;: \&quot;-34.4618\&quot;\n      }\n    },\n    \&quot;phone\&quot;: \&quot;010-692-6593 x09125\&quot;,\n    \&quot;website\&quot;: \&quot;anastasia.net\&quot;,\n    \&quot;company\&quot;: {\n      \&quot;name\&quot;: \&quot;Deckow-Crist\&quot;,\n      \&quot;catchPhrase\&quot;: \&quot;Proactive didactic contingency\&quot;,\n      \&quot;bs\&quot;: \&quot;synergize scalable supply-chains\&quot;\n    }\n  }&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>94fc4787-f184-44f2-90dc-4a6c3306195d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
WS.verifyElementPropertyValue(response, 'email', 'Shanna@melissa.tv')
WS.verifyElementPropertyValue(response, 'address.zipcode', '90566-7771')
WS.verifyElementPropertyValue(response, 'address.geo.lat', '-43.9509')
WS.verifyElementPropertyValue(response, 'address.geo.lng', '-34.4618')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
