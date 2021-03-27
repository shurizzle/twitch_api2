initSidebarItems({"enum":[["BodyError","Errors that can happen when creating a body"],["ClientRequestError","Errors for [`HelixClient::req_get`] and similar functions."],["CreateRequestError","Could not create request"],["HelixRequestDeleteError","Could not parse DELETE response"],["HelixRequestGetError","Could not parse GET response"],["HelixRequestPostError","Could not parse POST response"],["HelixRequestPutError","Could not parse PUT response"],["InvalidUri","Errors that can happen when creating [`http::Uri`] for [`Request`]"],["SerializeError","Errors from the query serializer"]],"mod":[["bits","Helix endpoints regarding bits"],["channels","Helix endpoints regarding channels"],["clips","Helix endpoints regarding clips"],["eventsub","Helix endpoints regarding EventSub"],["games","Helix endpoints regarding games"],["hypetrain","Helix endpoints regarding hype trains"],["moderation","Helix endpoints regarding moderation"],["points","Helix endpoints regarding channel points/redeems"],["search","Helix endpoints regarding search"],["streams","Helix endpoints regarding streams"],["subscriptions","Helix endpoints regarding subscriptions"],["tags","Helix endpoints regarding tags"],["teams","Helix endpoints regarding subscriptions"],["users","Helix endpoints regarding users"],["videos","Helix endpoints regarding videos"],["webhooks","Endpoints and topics for webhooks"]],"struct":[["EmptyBody","An empty body."],["HelixClient","Client for Helix or the New Twitch API"],["HelixRequestPatchError","helix returned error {status:?}: {message:?} when calling `PATCH {uri}` with a body"],["Response","Response retrieved from endpoint. Data is the type in [`Request::Response`]"]],"trait":[["HelixRequestBody","Create a body. Used for specializing request bodies"],["Paginated","Request can be paginated with a cursor"],["Request","A request is a Twitch endpoint, see New Twitch API reference"],["RequestDelete","Helix endpoint DELETEs information"],["RequestGet","Helix endpoint GETs information"],["RequestPatch","Helix endpoint PATCHs information"],["RequestPost","Helix endpoint POSTs information"],["RequestPut","Helix endpoint PUTs information"]],"type":[["Cursor","A cursor is a pointer to the current “page” in the twitch api pagination"]]});